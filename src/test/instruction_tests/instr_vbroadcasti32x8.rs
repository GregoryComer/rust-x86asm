use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 91, 36, 158], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 371325171, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 91, 12, 197, 243, 248, 33, 22], OperandSize::Qword)
}

