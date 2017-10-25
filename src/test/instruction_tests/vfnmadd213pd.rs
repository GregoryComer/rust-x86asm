use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 172, 228], OperandSize::Dword)
}

fn vfnmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1505055062, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 172, 60, 133, 86, 81, 181, 89], OperandSize::Dword)
}

fn vfnmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 172, 235], OperandSize::Qword)
}

fn vfnmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RAX, 905581332, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 172, 160, 20, 19, 250, 53], OperandSize::Qword)
}

fn vfnmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 172, 211], OperandSize::Dword)
}

fn vfnmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1435744353, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 172, 60, 245, 97, 184, 147, 85], OperandSize::Dword)
}

fn vfnmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 172, 244], OperandSize::Qword)
}

fn vfnmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 172, 44, 64], OperandSize::Qword)
}

fn vfnmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 172, 221], OperandSize::Dword)
}

fn vfnmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 1564993418, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 172, 139, 138, 231, 71, 93], OperandSize::Dword)
}

fn vfnmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 1689825931, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 154, 172, 150, 139, 178, 184, 100], OperandSize::Dword)
}

fn vfnmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 253, 131, 172, 221], OperandSize::Qword)
}

fn vfnmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 189, 129, 172, 4, 184], OperandSize::Qword)
}

fn vfnmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1557425898, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 149, 147, 172, 60, 149, 234, 110, 212, 92], OperandSize::Qword)
}

fn vfnmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 172, 236], OperandSize::Dword)
}

fn vfnmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 140464680, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 172, 172, 28, 253, 40, 82, 95, 8], OperandSize::Dword)
}

fn vfnmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 382191907, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 186, 172, 28, 221, 35, 201, 199, 22], OperandSize::Dword)
}

fn vfnmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 141, 166, 172, 217], OperandSize::Qword)
}

fn vfnmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 165, 161, 172, 58], OperandSize::Qword)
}

fn vfnmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 213, 185, 172, 57], OperandSize::Qword)
}

fn vfnmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 221, 172, 197], OperandSize::Dword)
}

fn vfnmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 503498789, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 204, 172, 168, 37, 200, 2, 30], OperandSize::Dword)
}

fn vfnmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EBX, 542230804, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 218, 172, 147, 20, 201, 81, 32], OperandSize::Dword)
}

fn vfnmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 253, 178, 172, 200], OperandSize::Qword)
}

fn vfnmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 157, 207, 172, 44, 202], OperandSize::Qword)
}

fn vfnmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 165, 221, 172, 36, 127], OperandSize::Qword)
}

