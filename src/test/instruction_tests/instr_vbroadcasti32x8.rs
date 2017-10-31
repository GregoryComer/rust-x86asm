use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 91, 1], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1510330843, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 203, 91, 132, 159, 219, 209, 5, 90], OperandSize::Qword)
}

