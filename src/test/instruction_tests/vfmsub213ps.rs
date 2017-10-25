use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 170, 232], OperandSize::Dword)
}

fn vfmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 170, 52, 144], OperandSize::Dword)
}

fn vfmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 170, 196], OperandSize::Qword)
}

fn vfmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 913450757, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 170, 44, 205, 5, 39, 114, 54], OperandSize::Qword)
}

fn vfmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 170, 242], OperandSize::Dword)
}

fn vfmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 170, 36, 121], OperandSize::Dword)
}

fn vfmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 170, 251], OperandSize::Qword)
}

fn vfmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 2108500399, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 170, 12, 253, 175, 41, 173, 125], OperandSize::Qword)
}

fn vfmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 142, 170, 243], OperandSize::Dword)
}

fn vfmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 170, 60, 144], OperandSize::Dword)
}

fn vfmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 157, 170, 20, 87], OperandSize::Dword)
}

fn vfmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 85, 137, 170, 213], OperandSize::Qword)
}

fn vfmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RDI, 1229888693, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 53, 142, 170, 167, 181, 156, 78, 73], OperandSize::Qword)
}

fn vfmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 85, 146, 170, 20, 210], OperandSize::Qword)
}

fn vfmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 170, 233], OperandSize::Dword)
}

fn vfmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 1096545605, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 170, 168, 69, 245, 91, 65], OperandSize::Dword)
}

fn vfmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 61029606, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 191, 170, 186, 230, 60, 163, 3], OperandSize::Dword)
}

fn vfmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 93, 167, 170, 246], OperandSize::Qword)
}

fn vfmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RBX, 150268907, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 77, 172, 170, 139, 235, 235, 244, 8], OperandSize::Qword)
}

fn vfmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 85, 187, 170, 36, 209], OperandSize::Qword)
}

fn vfmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 186, 170, 196], OperandSize::Dword)
}

fn vfmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1696110784, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 201, 170, 12, 253, 192, 152, 24, 101], OperandSize::Dword)
}

fn vfmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 221, 170, 19], OperandSize::Dword)
}

fn vfmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 117, 255, 170, 208], OperandSize::Qword)
}

fn vfmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 69, 195, 170, 2], OperandSize::Qword)
}

fn vfmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 69, 217, 170, 52, 218], OperandSize::Qword)
}

