use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vdivpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 94, 211], OperandSize::Dword)
}

fn vdivpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 182795315, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 94, 163, 51, 60, 229, 10], OperandSize::Dword)
}

fn vdivpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 94, 226], OperandSize::Qword)
}

fn vdivpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 94, 18], OperandSize::Qword)
}

fn vdivpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 94, 197], OperandSize::Dword)
}

fn vdivpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 964051389, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 94, 60, 157, 189, 65, 118, 57], OperandSize::Dword)
}

fn vdivpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 94, 238], OperandSize::Qword)
}

fn vdivpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RSI, 1409425416, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 94, 150, 8, 32, 2, 84], OperandSize::Qword)
}

fn vdivpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 143, 94, 245], OperandSize::Dword)
}

fn vdivpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1401770996, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 137, 94, 36, 189, 244, 83, 141, 83], OperandSize::Dword)
}

fn vdivpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1233199839, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 159, 94, 28, 149, 223, 34, 129, 73], OperandSize::Dword)
}

fn vdivpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 229, 137, 94, 231], OperandSize::Qword)
}

fn vdivpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RSI, 968498943, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 173, 135, 94, 174, 255, 30, 186, 57], OperandSize::Qword)
}

fn vdivpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 57223710, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 173, 151, 94, 28, 77, 30, 42, 105, 3], OperandSize::Qword)
}

fn vdivpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 173, 94, 215], OperandSize::Dword)
}

fn vdivpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 161596542, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 170, 94, 172, 119, 126, 196, 161, 9], OperandSize::Dword)
}

fn vdivpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 191, 94, 36, 113], OperandSize::Dword)
}

fn vdivpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 189, 161, 94, 209], OperandSize::Qword)
}

fn vdivpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 170, 94, 4, 243], OperandSize::Qword)
}

fn vdivpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1620155824, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 149, 180, 94, 36, 253, 176, 157, 145, 96], OperandSize::Qword)
}

fn vdivpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 218, 94, 209], OperandSize::Dword)
}

fn vdivpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 847773396, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 204, 94, 168, 212, 254, 135, 50], OperandSize::Dword)
}

fn vdivpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 220, 94, 36, 248], OperandSize::Dword)
}

fn vdivpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 133, 155, 94, 212], OperandSize::Qword)
}

fn vdivpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 173, 206, 94, 20, 208], OperandSize::Qword)
}

fn vdivpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RCX, 1718012649, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 218, 94, 137, 233, 202, 102, 102], OperandSize::Qword)
}

