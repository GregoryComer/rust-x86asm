use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vandnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 85, 233], OperandSize::Dword)
}

fn vandnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 733507314, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 85, 161, 242, 110, 184, 43], OperandSize::Dword)
}

fn vandnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 85, 242], OperandSize::Qword)
}

fn vandnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 85, 32], OperandSize::Qword)
}

fn vandnps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 85, 212], OperandSize::Dword)
}

fn vandnps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 986944851, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 85, 140, 67, 83, 149, 211, 58], OperandSize::Dword)
}

fn vandnps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 85, 241], OperandSize::Qword)
}

fn vandnps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 85, 30], OperandSize::Qword)
}

fn vandnps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 85, 212], OperandSize::Dword)
}

fn vandnps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1086933484, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 84, 139, 85, 132, 113, 236, 73, 201, 64], OperandSize::Dword)
}

fn vandnps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 2054078976, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 116, 155, 85, 156, 195, 0, 194, 110, 122], OperandSize::Dword)
}

fn vandnps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 84, 130, 85, 246], OperandSize::Qword)
}

fn vandnps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1045922503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 60, 141, 85, 44, 149, 199, 130, 87, 62], OperandSize::Qword)
}

fn vandnps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 4, 158, 85, 8], OperandSize::Qword)
}

fn vandnps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 100, 175, 85, 216], OperandSize::Dword)
}

fn vandnps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1575305124, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 116, 170, 85, 60, 197, 164, 63, 229, 93], OperandSize::Dword)
}

fn vandnps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 925534125, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 191, 85, 177, 173, 135, 42, 55], OperandSize::Dword)
}

fn vandnps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 116, 169, 85, 244], OperandSize::Qword)
}

fn vandnps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 174, 85, 9], OperandSize::Qword)
}

fn vandnps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 124, 179, 85, 60, 246], OperandSize::Qword)
}

fn vandnps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 202, 85, 213], OperandSize::Dword)
}

fn vandnps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 2117548171, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 205, 85, 148, 219, 139, 56, 55, 126], OperandSize::Dword)
}

fn vandnps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 222, 85, 44, 190], OperandSize::Dword)
}

fn vandnps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 116, 206, 85, 213], OperandSize::Qword)
}

fn vandnps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 108, 203, 85, 60, 208], OperandSize::Qword)
}

fn vandnps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 213, 85, 28, 200], OperandSize::Qword)
}

