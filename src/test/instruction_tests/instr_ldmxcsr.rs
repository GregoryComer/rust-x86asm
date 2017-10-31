use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectDisplaced(EDI, 887257178, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 151, 90, 120, 226, 52], OperandSize::Dword)
}

#[test]
fn ldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1634046450, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 20, 149, 242, 145, 101, 97], OperandSize::Qword)
}

