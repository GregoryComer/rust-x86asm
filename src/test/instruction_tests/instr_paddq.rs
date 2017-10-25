use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 216], OperandSize::Dword)
}

#[test]
fn paddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1054584035, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 4, 93, 227, 172, 219, 62], OperandSize::Dword)
}

#[test]
fn paddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 238], OperandSize::Qword)
}

#[test]
fn paddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1977329487, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 52, 189, 79, 167, 219, 117], OperandSize::Qword)
}

#[test]
fn paddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 198], OperandSize::Dword)
}

#[test]
fn paddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 20, 151], OperandSize::Dword)
}

#[test]
fn paddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 217], OperandSize::Qword)
}

#[test]
fn paddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 845320913, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 184, 209, 146, 98, 50], OperandSize::Qword)
}

