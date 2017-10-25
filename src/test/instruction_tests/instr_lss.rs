use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(BP)), operand2: Some(Indirect(BX, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 47], OperandSize::Word)
}

#[test]
fn lss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(SI)), operand2: Some(Indirect(EAX, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 48], OperandSize::Dword)
}

#[test]
fn lss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1798706458, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 36, 149, 26, 21, 54, 107], OperandSize::Qword)
}

#[test]
fn lss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(EDI, 917684293, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 151, 69, 192, 178, 54], OperandSize::Dword)
}

#[test]
fn lss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(RDI, 957947274, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 143, 138, 29, 25, 57], OperandSize::Qword)
}

#[test]
fn lss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 178, 60, 71], OperandSize::Qword)
}

