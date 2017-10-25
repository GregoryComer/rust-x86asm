use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsf_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 219], OperandSize::Word)
}

#[test]
fn bsf_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 25], OperandSize::Word)
}

#[test]
fn bsf_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 215], OperandSize::Dword)
}

#[test]
fn bsf_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EAX, 1685362166, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 152, 246, 149, 116, 100], OperandSize::Dword)
}

#[test]
fn bsf_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 214], OperandSize::Qword)
}

#[test]
fn bsf_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1277348183, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 28, 133, 87, 201, 34, 76], OperandSize::Qword)
}

#[test]
fn bsf_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 252], OperandSize::Word)
}

#[test]
fn bsf_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 58], OperandSize::Word)
}

#[test]
fn bsf_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 233], OperandSize::Dword)
}

#[test]
fn bsf_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1462603532, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 52, 77, 12, 143, 45, 87], OperandSize::Dword)
}

#[test]
fn bsf_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 234], OperandSize::Qword)
}

#[test]
fn bsf_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 798503200, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 28, 245, 32, 49, 152, 47], OperandSize::Qword)
}

#[test]
fn bsf_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 239], OperandSize::Qword)
}

#[test]
fn bsf_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 59], OperandSize::Qword)
}

