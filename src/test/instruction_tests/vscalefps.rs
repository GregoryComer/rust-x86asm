use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vscalefps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 44, 255], OperandSize::Dword)
}

fn vscalefps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 44, 20, 150], OperandSize::Dword)
}

fn vscalefps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 779969808, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 153, 44, 172, 183, 16, 101, 125, 46], OperandSize::Dword)
}

fn vscalefps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 117, 137, 44, 215], OperandSize::Qword)
}

fn vscalefps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 37, 131, 44, 54], OperandSize::Qword)
}

fn vscalefps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 109, 147, 44, 4, 130], OperandSize::Qword)
}

fn vscalefps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 44, 198], OperandSize::Dword)
}

fn vscalefps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 647151102, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 44, 60, 77, 254, 189, 146, 38], OperandSize::Dword)
}

fn vscalefps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 69858843, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 187, 44, 156, 67, 27, 246, 41, 4], OperandSize::Dword)
}

fn vscalefps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 13, 170, 44, 204], OperandSize::Qword)
}

fn vscalefps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 609767637, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 109, 173, 44, 132, 81, 213, 80, 88, 36], OperandSize::Qword)
}

fn vscalefps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 61, 191, 44, 52, 195], OperandSize::Qword)
}

fn vscalefps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 190, 44, 213], OperandSize::Dword)
}

fn vscalefps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ESI, 1881317008, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 44, 174, 144, 158, 34, 112], OperandSize::Dword)
}

fn vscalefps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 221, 44, 20, 136], OperandSize::Dword)
}

fn vscalefps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 21, 244, 44, 240], OperandSize::Qword)
}

fn vscalefps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM13)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 21, 206, 44, 51], OperandSize::Qword)
}

fn vscalefps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RSI, 852751900, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 37, 211, 44, 182, 28, 246, 211, 50], OperandSize::Qword)
}

