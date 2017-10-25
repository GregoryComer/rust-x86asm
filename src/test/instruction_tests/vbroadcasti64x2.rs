use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcasti64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 90, 38], OperandSize::Dword)
}

fn vbroadcasti64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 180670033, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 90, 28, 117, 81, 206, 196, 10], OperandSize::Qword)
}

fn vbroadcasti64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 90, 4, 216], OperandSize::Dword)
}

fn vbroadcasti64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 90, 15], OperandSize::Qword)
}

