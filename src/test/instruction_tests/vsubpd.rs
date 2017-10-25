use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 92, 214], OperandSize::Dword)
}

fn vsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 92, 12, 95], OperandSize::Dword)
}

fn vsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 92, 231], OperandSize::Qword)
}

fn vsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 816380213, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 92, 143, 53, 249, 168, 48], OperandSize::Qword)
}

fn vsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 92, 235], OperandSize::Dword)
}

fn vsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 521110633, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 92, 167, 105, 132, 15, 31], OperandSize::Dword)
}

fn vsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 92, 207], OperandSize::Qword)
}

fn vsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 92, 18], OperandSize::Qword)
}

fn vsubpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 92, 243], OperandSize::Dword)
}

fn vsubpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 143, 92, 46], OperandSize::Dword)
}

fn vsubpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 156, 92, 31], OperandSize::Dword)
}

fn vsubpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 165, 133, 92, 205], OperandSize::Qword)
}

fn vsubpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 133, 92, 52, 89], OperandSize::Qword)
}

fn vsubpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 251500598, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 229, 150, 92, 140, 201, 54, 152, 253, 14], OperandSize::Qword)
}

fn vsubpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 173, 92, 225], OperandSize::Dword)
}

fn vsubpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 173, 92, 36, 112], OperandSize::Dword)
}

fn vsubpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 92, 4, 251], OperandSize::Dword)
}

fn vsubpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 181, 161, 92, 244], OperandSize::Qword)
}

fn vsubpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 157, 175, 92, 15], OperandSize::Qword)
}

fn vsubpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 197, 182, 92, 20, 151], OperandSize::Qword)
}

fn vsubpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 255, 92, 208], OperandSize::Dword)
}

fn vsubpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 938291878, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 201, 92, 44, 221, 166, 50, 237, 55], OperandSize::Dword)
}

fn vsubpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1531605390, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 220, 92, 52, 77, 142, 113, 74, 91], OperandSize::Dword)
}

fn vsubpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 141, 155, 92, 217], OperandSize::Qword)
}

fn vsubpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1602167974, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 205, 197, 92, 4, 213, 166, 36, 127, 95], OperandSize::Qword)
}

fn vsubpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 215, 92, 58], OperandSize::Qword)
}

