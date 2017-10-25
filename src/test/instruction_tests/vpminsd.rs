use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 57, 192], OperandSize::Dword)
}

fn vpminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1149378970, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 57, 20, 205, 154, 33, 130, 68], OperandSize::Dword)
}

fn vpminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 57, 242], OperandSize::Qword)
}

fn vpminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 1527413033, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 57, 138, 41, 121, 10, 91], OperandSize::Qword)
}

fn vpminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 57, 220], OperandSize::Dword)
}

fn vpminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDI, 598240869, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 57, 135, 101, 110, 168, 35], OperandSize::Dword)
}

fn vpminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 57, 240], OperandSize::Qword)
}

fn vpminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 57, 28, 200], OperandSize::Qword)
}

fn vpminsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 57, 230], OperandSize::Dword)
}

fn vpminsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 57, 60, 179], OperandSize::Dword)
}

fn vpminsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1279128370, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 157, 57, 180, 187, 50, 243, 61, 76], OperandSize::Dword)
}

fn vpminsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 13, 141, 57, 196], OperandSize::Qword)
}

fn vpminsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RBX, 1161300040, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 45, 130, 57, 179, 72, 8, 56, 69], OperandSize::Qword)
}

fn vpminsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 117, 145, 57, 60, 182], OperandSize::Qword)
}

fn vpminsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 174, 57, 237], OperandSize::Dword)
}

fn vpminsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 1170579469, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 57, 166, 13, 160, 197, 69], OperandSize::Dword)
}

fn vpminsd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 187, 57, 8], OperandSize::Dword)
}

fn vpminsd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 125, 171, 57, 218], OperandSize::Qword)
}

fn vpminsd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 451449052, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 45, 174, 57, 156, 203, 220, 144, 232, 26], OperandSize::Qword)
}

fn vpminsd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 21, 180, 57, 28, 83], OperandSize::Qword)
}

fn vpminsd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 57, 233], OperandSize::Dword)
}

fn vpminsd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 403993434, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 202, 57, 132, 246, 90, 115, 20, 24], OperandSize::Dword)
}

fn vpminsd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 22442484, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 222, 57, 36, 221, 244, 113, 86, 1], OperandSize::Dword)
}

fn vpminsd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 5, 205, 57, 193], OperandSize::Qword)
}

fn vpminsd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 194, 57, 12, 179], OperandSize::Qword)
}

fn vpminsd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 29, 211, 57, 52, 113], OperandSize::Qword)
}

