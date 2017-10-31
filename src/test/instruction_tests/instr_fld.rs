use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(BP, 28889, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 134, 217, 112], OperandSize::Word)
}

#[test]
fn fld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(EBX, 1097464985, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 131, 153, 252, 105, 65], OperandSize::Dword)
}

#[test]
fn fld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1706788638, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 4, 205, 30, 135, 187, 101], OperandSize::Qword)
}

#[test]
fn fld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 194], OperandSize::Word)
}

#[test]
fn fld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 199], OperandSize::Dword)
}

#[test]
fn fld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 193], OperandSize::Qword)
}

#[test]
fn fld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(DI, 96, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 109, 96], OperandSize::Word)
}

#[test]
fn fld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledDisplaced(EDX, Two, 145087618, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 44, 85, 130, 220, 165, 8], OperandSize::Dword)
}

#[test]
fn fld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1823102518, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 172, 190, 54, 86, 170, 108], OperandSize::Qword)
}

#[test]
fn fld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 32405, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 129, 149, 126], OperandSize::Word)
}

#[test]
fn fld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(ECX, 80305532, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 129, 124, 93, 201, 4], OperandSize::Dword)
}

#[test]
fn fld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 36881779, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 132, 146, 115, 197, 50, 2], OperandSize::Qword)
}

