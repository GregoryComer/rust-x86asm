use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcasti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 90, 38], OperandSize::Dword)
}

fn vbroadcasti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 281232932, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 173, 90, 4, 245, 36, 70, 195, 16], OperandSize::Qword)
}

fn vbroadcasti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 502006223, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 90, 172, 155, 207, 1, 236, 29], OperandSize::Dword)
}

fn vbroadcasti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectDisplaced(RBX, 1345660638, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 206, 90, 155, 222, 38, 53, 80], OperandSize::Qword)
}

