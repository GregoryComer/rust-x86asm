use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vminps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 93, 229], OperandSize::Dword)
}

fn vminps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 1961118536, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 93, 150, 72, 75, 228, 116], OperandSize::Dword)
}

fn vminps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 93, 211], OperandSize::Qword)
}

fn vminps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 93, 15], OperandSize::Qword)
}

fn vminps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 93, 237], OperandSize::Dword)
}

fn vminps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 255970504, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 93, 177, 200, 204, 65, 15], OperandSize::Dword)
}

fn vminps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 93, 246], OperandSize::Qword)
}

fn vminps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 93, 7], OperandSize::Qword)
}

fn vminps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 137, 93, 255], OperandSize::Dword)
}

fn vminps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 100, 143, 93, 12, 159], OperandSize::Dword)
}

fn vminps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 354399621, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 159, 93, 172, 195, 133, 181, 31, 21], OperandSize::Dword)
}

fn vminps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 76, 129, 93, 199], OperandSize::Qword)
}

fn vminps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 143, 93, 28, 210], OperandSize::Qword)
}

fn vminps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RCX, 440493165, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 28, 155, 93, 153, 109, 100, 65, 26], OperandSize::Qword)
}

fn vminps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 84, 171, 93, 216], OperandSize::Dword)
}

fn vminps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 661540227, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 84, 174, 93, 152, 131, 77, 110, 39], OperandSize::Dword)
}

fn vminps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 404331680, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 191, 93, 140, 187, 160, 156, 25, 24], OperandSize::Dword)
}

fn vminps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 4, 165, 93, 226], OperandSize::Qword)
}

fn vminps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 788303251, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 170, 93, 28, 133, 147, 141, 252, 46], OperandSize::Qword)
}

fn vminps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 44, 182, 93, 28, 79], OperandSize::Qword)
}

fn vminps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 157, 93, 208], OperandSize::Dword)
}

fn vminps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 100, 201, 93, 36, 94], OperandSize::Dword)
}

fn vminps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 217, 93, 20, 66], OperandSize::Dword)
}

fn vminps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 20, 159, 93, 238], OperandSize::Qword)
}

fn vminps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 100, 199, 93, 52, 193], OperandSize::Qword)
}

fn vminps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 76, 211, 93, 16], OperandSize::Qword)
}

