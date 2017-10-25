use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcasti32x8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 91, 12, 198], OperandSize::Dword)
}

fn vbroadcasti32x8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1016714206, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 91, 4, 141, 222, 211, 153, 60], OperandSize::Qword)
}

