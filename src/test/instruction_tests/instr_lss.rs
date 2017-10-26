use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 11], OperandSize::Word)
}

#[test]
fn lss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EAX, 1096637992, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 152, 40, 94, 93, 65], OperandSize::Dword)
}

#[test]
fn lss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(RAX, 1638742012, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 144, 252, 55, 173, 97], OperandSize::Qword)
}

#[test]
fn lss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 263663467, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 148, 154, 107, 47, 183, 15], OperandSize::Dword)
}

#[test]
fn lss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RAX, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 24], OperandSize::Qword)
}

#[test]
fn lss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RCX, 740358039, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 178, 185, 151, 247, 32, 44], OperandSize::Qword)
}

