use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 174, 230], OperandSize::Dword)
}

fn vfnmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 174, 20, 70], OperandSize::Dword)
}

fn vfnmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 174, 220], OperandSize::Qword)
}

fn vfnmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 2077807279, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 174, 140, 246, 175, 210, 216, 123], OperandSize::Qword)
}

fn vfnmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 174, 253], OperandSize::Dword)
}

fn vfnmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1629701410, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 174, 172, 114, 34, 69, 35, 97], OperandSize::Dword)
}

fn vfnmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 174, 209], OperandSize::Qword)
}

fn vfnmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 312387147, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 174, 36, 221, 75, 166, 158, 18], OperandSize::Qword)
}

fn vfnmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 137, 174, 249], OperandSize::Dword)
}

fn vfnmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 137, 174, 58], OperandSize::Dword)
}

fn vfnmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1690188804, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 154, 174, 164, 215, 4, 60, 190, 100], OperandSize::Dword)
}

fn vfnmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 173, 143, 174, 197], OperandSize::Qword)
}

fn vfnmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RSI, 57760896, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 149, 130, 174, 174, 128, 92, 113, 3], OperandSize::Qword)
}

fn vfnmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 181, 149, 174, 17], OperandSize::Qword)
}

fn vfnmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 174, 226], OperandSize::Dword)
}

fn vfnmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1709111897, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 174, 129, 89, 250, 222, 101], OperandSize::Dword)
}

fn vfnmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 2027383818, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 185, 174, 140, 178, 10, 108, 215, 120], OperandSize::Dword)
}

fn vfnmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 189, 163, 174, 250], OperandSize::Qword)
}

fn vfnmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1415574924, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 221, 174, 174, 12, 117, 140, 245, 95, 84], OperandSize::Qword)
}

fn vfnmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 197, 189, 174, 62], OperandSize::Qword)
}

fn vfnmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 159, 174, 217], OperandSize::Dword)
}

fn vfnmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 999259452, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 206, 174, 131, 60, 125, 143, 59], OperandSize::Dword)
}

fn vfnmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 763600639, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 217, 174, 156, 206, 255, 158, 131, 45], OperandSize::Dword)
}

fn vfnmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 253, 220, 174, 237], OperandSize::Qword)
}

fn vfnmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 723722295, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 149, 204, 174, 188, 194, 55, 32, 35, 43], OperandSize::Qword)
}

fn vfnmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 99300147, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 165, 215, 174, 148, 134, 51, 51, 235, 5], OperandSize::Qword)
}

