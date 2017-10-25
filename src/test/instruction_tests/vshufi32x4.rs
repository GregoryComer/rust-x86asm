use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vshufi32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 205, 67, 225, 30], OperandSize::Dword)
}

fn vshufi32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1659030697, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 117, 201, 67, 12, 181, 169, 204, 226, 98, 127], OperandSize::Dword)
}

fn vshufi32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 13702175, Some(OperandSize::Dword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 77, 223, 67, 52, 77, 31, 20, 209, 0, 100], OperandSize::Dword)
}

fn vshufi32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM18)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 163, 53, 198, 67, 250, 15], OperandSize::Qword)
}

fn vshufi32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 45, 196, 67, 2, 68], OperandSize::Qword)
}

fn vshufi32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RAX, 1279524468, Some(OperandSize::Dword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 13, 211, 67, 168, 116, 254, 67, 76, 33], OperandSize::Qword)
}

