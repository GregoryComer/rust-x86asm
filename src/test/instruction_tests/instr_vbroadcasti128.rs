use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 597972456, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 172, 142, 232, 85, 164, 35], OperandSize::Dword)
}

#[test]
fn vbroadcasti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 12, 217], OperandSize::Qword)
}

