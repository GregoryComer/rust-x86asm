use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vblendmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 101, 236], OperandSize::Dword)
}

fn vblendmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 74789883, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 101, 28, 245, 251, 51, 117, 4], OperandSize::Dword)
}

fn vblendmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 830594311, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 153, 101, 60, 77, 7, 221, 129, 49], OperandSize::Dword)
}

fn vblendmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 189, 131, 101, 229], OperandSize::Qword)
}

fn vblendmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 407890240, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 133, 101, 164, 218, 64, 233, 79, 24], OperandSize::Qword)
}

fn vblendmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 165, 145, 101, 60, 152], OperandSize::Qword)
}

fn vblendmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 101, 252], OperandSize::Dword)
}

fn vblendmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 827743199, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 101, 4, 77, 223, 91, 86, 49], OperandSize::Dword)
}

fn vblendmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 186, 101, 4, 214], OperandSize::Dword)
}

fn vblendmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 245, 163, 101, 218], OperandSize::Qword)
}

fn vblendmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 197, 173, 101, 31], OperandSize::Qword)
}

fn vblendmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 197, 179, 101, 36, 119], OperandSize::Qword)
}

fn vblendmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 204, 101, 244], OperandSize::Dword)
}

fn vblendmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1703107060, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 101, 12, 245, 244, 89, 131, 101], OperandSize::Dword)
}

fn vblendmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1404040537, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 101, 180, 79, 89, 245, 175, 83], OperandSize::Dword)
}

fn vblendmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 197, 206, 101, 250], OperandSize::Qword)
}

fn vblendmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RDX, 792943896, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 157, 205, 101, 170, 24, 93, 67, 47], OperandSize::Qword)
}

fn vblendmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 223, 101, 60, 67], OperandSize::Qword)
}

