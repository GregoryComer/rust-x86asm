use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vxorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 87, 217], OperandSize::Dword)
}

fn vxorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 410914796, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 87, 60, 141, 236, 15, 126, 24], OperandSize::Dword)
}

fn vxorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 87, 232], OperandSize::Qword)
}

fn vxorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RCX, 1359318335, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 87, 137, 63, 141, 5, 81], OperandSize::Qword)
}

fn vxorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 87, 235], OperandSize::Dword)
}

fn vxorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1376832625, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 87, 36, 141, 113, 204, 16, 82], OperandSize::Dword)
}

fn vxorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 87, 213], OperandSize::Qword)
}

fn vxorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1276353809, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 87, 164, 247, 17, 157, 19, 76], OperandSize::Qword)
}

fn vxorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 143, 87, 248], OperandSize::Dword)
}

fn vxorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1440786934, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 139, 87, 4, 77, 246, 169, 224, 85], OperandSize::Dword)
}

fn vxorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1512362588, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 159, 87, 44, 93, 92, 210, 36, 90], OperandSize::Dword)
}

fn vxorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 100, 130, 87, 218], OperandSize::Qword)
}

fn vxorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 92, 134, 87, 6], OperandSize::Qword)
}

fn vxorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1931768918, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 84, 154, 87, 36, 77, 86, 116, 36, 115], OperandSize::Qword)
}

fn vxorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 173, 87, 211], OperandSize::Dword)
}

fn vxorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 84, 169, 87, 32], OperandSize::Dword)
}

fn vxorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1681262948, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 185, 87, 12, 157, 100, 9, 54, 100], OperandSize::Dword)
}

fn vxorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 28, 169, 87, 214], OperandSize::Qword)
}

fn vxorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 919975935, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 100, 172, 87, 52, 69, 255, 183, 213, 54], OperandSize::Qword)
}

fn vxorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RAX, 1602884529, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 191, 87, 152, 177, 19, 138, 95], OperandSize::Qword)
}

fn vxorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 205, 87, 195], OperandSize::Dword)
}

fn vxorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 50829251, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 76, 207, 87, 188, 90, 195, 151, 7, 3], OperandSize::Dword)
}

fn vxorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 154511886, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 221, 87, 60, 221, 14, 170, 53, 9], OperandSize::Dword)
}

fn vxorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 100, 205, 87, 245], OperandSize::Qword)
}

fn vxorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1677801878, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 108, 201, 87, 20, 93, 150, 57, 1, 100], OperandSize::Qword)
}

fn vxorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RAX, 383833444, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 76, 218, 87, 152, 100, 213, 224, 22], OperandSize::Qword)
}

