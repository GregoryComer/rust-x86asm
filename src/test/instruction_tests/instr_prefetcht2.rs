use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectDisplaced(SI, 28588, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 156, 172, 111], OperandSize::Word)
}

#[test]
fn prefetcht2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 26], OperandSize::Dword)
}

#[test]
fn prefetcht2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectScaledDisplaced(RDX, Four, 985760127, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 28, 149, 127, 129, 193, 58], OperandSize::Qword)
}

