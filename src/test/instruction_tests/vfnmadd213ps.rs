use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 172, 232], OperandSize::Dword)
}

fn vfnmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 172, 60, 123], OperandSize::Dword)
}

fn vfnmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 172, 197], OperandSize::Qword)
}

fn vfnmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 172, 44, 182], OperandSize::Qword)
}

fn vfnmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 172, 240], OperandSize::Dword)
}

fn vfnmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 956107140, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 172, 179, 132, 9, 253, 56], OperandSize::Dword)
}

fn vfnmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 172, 221], OperandSize::Qword)
}

fn vfnmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 172, 28, 155], OperandSize::Qword)
}

fn vfnmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 172, 231], OperandSize::Dword)
}

fn vfnmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1904187299, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 172, 188, 78, 163, 151, 127, 113], OperandSize::Dword)
}

fn vfnmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 158, 172, 18], OperandSize::Dword)
}

fn vfnmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 101, 129, 172, 202], OperandSize::Qword)
}

fn vfnmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1477332147, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 133, 172, 20, 149, 179, 76, 14, 88], OperandSize::Qword)
}

fn vfnmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 29605213, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 5, 148, 172, 172, 83, 93, 189, 195, 1], OperandSize::Qword)
}

fn vfnmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 172, 253], OperandSize::Dword)
}

fn vfnmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1510545469, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 172, 180, 83, 61, 24, 9, 90], OperandSize::Dword)
}

fn vfnmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 186, 172, 33], OperandSize::Dword)
}

fn vfnmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 85, 161, 172, 192], OperandSize::Qword)
}

fn vfnmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 172, 28, 179], OperandSize::Qword)
}

fn vfnmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1565677302, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 101, 187, 172, 156, 90, 246, 86, 82, 93], OperandSize::Qword)
}

fn vfnmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 249, 172, 196], OperandSize::Dword)
}

fn vfnmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 172, 0], OperandSize::Dword)
}

fn vfnmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 250356004, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 223, 172, 52, 221, 36, 33, 236, 14], OperandSize::Dword)
}

fn vfnmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 69, 245, 172, 235], OperandSize::Qword)
}

fn vfnmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 37, 202, 172, 58], OperandSize::Qword)
}

fn vfnmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 952018641, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 5, 210, 172, 140, 177, 209, 166, 190, 56], OperandSize::Qword)
}

