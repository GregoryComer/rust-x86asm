use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(DI, 24650, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 133, 74, 96], OperandSize::Word)
}

#[test]
fn fld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1571062532, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 132, 186, 4, 131, 164, 93], OperandSize::Dword)
}

#[test]
fn fld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 1], OperandSize::Qword)
}

#[test]
fn fld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 197], OperandSize::Word)
}

#[test]
fn fld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 195], OperandSize::Dword)
}

#[test]
fn fld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 195], OperandSize::Qword)
}

#[test]
fn fld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 100, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 106, 100], OperandSize::Word)
}

#[test]
fn fld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledDisplaced(ECX, Four, 830905206, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 44, 141, 118, 155, 134, 49], OperandSize::Dword)
}

#[test]
fn fld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Indirect(RSI, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 46], OperandSize::Qword)
}

#[test]
fn fld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 7], OperandSize::Word)
}

#[test]
fn fld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 4, 193], OperandSize::Dword)
}

#[test]
fn fld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(RCX, 1132780298, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 129, 10, 219, 132, 67], OperandSize::Qword)
}

