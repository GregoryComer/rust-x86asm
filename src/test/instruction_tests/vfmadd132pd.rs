use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 152, 221], OperandSize::Dword)
}

fn vfmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1334878490, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 152, 148, 131, 26, 161, 144, 79], OperandSize::Dword)
}

fn vfmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 152, 227], OperandSize::Qword)
}

fn vfmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 532052719, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 152, 4, 69, 239, 122, 182, 31], OperandSize::Qword)
}

fn vfmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 152, 205], OperandSize::Dword)
}

fn vfmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1545096010, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 152, 156, 75, 74, 75, 24, 92], OperandSize::Dword)
}

fn vfmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 152, 228], OperandSize::Qword)
}

fn vfmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 152, 60, 147], OperandSize::Qword)
}

fn vfmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 152, 237], OperandSize::Dword)
}

fn vfmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 152, 60, 203], OperandSize::Dword)
}

fn vfmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 154, 152, 10], OperandSize::Dword)
}

fn vfmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 213, 135, 152, 203], OperandSize::Qword)
}

fn vfmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1070768274, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 140, 152, 12, 253, 146, 160, 210, 63], OperandSize::Qword)
}

fn vfmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 181, 157, 152, 60, 114], OperandSize::Qword)
}

fn vfmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 152, 208], OperandSize::Dword)
}

fn vfmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 152, 12, 218], OperandSize::Dword)
}

fn vfmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1554189229, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 185, 152, 60, 85, 173, 11, 163, 92], OperandSize::Dword)
}

fn vfmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 181, 174, 152, 238], OperandSize::Qword)
}

fn vfmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 958378439, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 141, 170, 152, 4, 69, 199, 177, 31, 57], OperandSize::Qword)
}

fn vfmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1280487246, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 149, 178, 152, 28, 133, 78, 175, 82, 76], OperandSize::Qword)
}

fn vfmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 218, 152, 251], OperandSize::Dword)
}

fn vfmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 203, 152, 12, 193], OperandSize::Dword)
}

fn vfmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 218, 152, 52, 187], OperandSize::Dword)
}

fn vfmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 229, 220, 152, 239], OperandSize::Qword)
}

fn vfmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RDI, 1988152611, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 165, 199, 152, 183, 35, 205, 128, 118], OperandSize::Qword)
}

fn vfmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 526294216, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 212, 152, 60, 77, 200, 156, 94, 31], OperandSize::Qword)
}

