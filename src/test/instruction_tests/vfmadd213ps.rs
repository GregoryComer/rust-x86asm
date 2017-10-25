use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 168, 241], OperandSize::Dword)
}

fn vfmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 901363791, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 168, 52, 141, 79, 184, 185, 53], OperandSize::Dword)
}

fn vfmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 168, 199], OperandSize::Qword)
}

fn vfmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 168, 52, 194], OperandSize::Qword)
}

fn vfmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 168, 249], OperandSize::Dword)
}

fn vfmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 516548638, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 168, 164, 113, 30, 232, 201, 30], OperandSize::Dword)
}

fn vfmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 168, 230], OperandSize::Qword)
}

fn vfmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RAX, 1323491921, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 168, 144, 81, 226, 226, 78], OperandSize::Qword)
}

fn vfmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 168, 207], OperandSize::Dword)
}

fn vfmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 2019653522, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 168, 188, 127, 146, 119, 97, 120], OperandSize::Dword)
}

fn vfmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 915178818, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 158, 168, 12, 141, 66, 133, 140, 54], OperandSize::Dword)
}

fn vfmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 21, 130, 168, 213], OperandSize::Qword)
}

fn vfmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RDX, 459487334, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 29, 137, 168, 146, 102, 56, 99, 27], OperandSize::Qword)
}

fn vfmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RDI, 374062865, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 61, 153, 168, 191, 17, 191, 75, 22], OperandSize::Qword)
}

fn vfmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 169, 168, 209], OperandSize::Dword)
}

fn vfmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 1222612298, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 169, 168, 164, 82, 74, 149, 223, 72], OperandSize::Dword)
}

fn vfmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 501907184, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 186, 168, 153, 240, 126, 234, 29], OperandSize::Dword)
}

fn vfmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 61, 163, 168, 212], OperandSize::Qword)
}

fn vfmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RAX, 1172546266, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 21, 163, 168, 144, 218, 162, 227, 69], OperandSize::Qword)
}

fn vfmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 45, 185, 168, 56], OperandSize::Qword)
}

fn vfmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 220, 168, 250], OperandSize::Dword)
}

fn vfmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 206, 168, 28, 115], OperandSize::Dword)
}

fn vfmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 809020194, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 220, 168, 44, 133, 34, 171, 56, 48], OperandSize::Dword)
}

fn vfmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 21, 223, 168, 221], OperandSize::Qword)
}

fn vfmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 348298180, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 61, 201, 168, 4, 77, 196, 155, 194, 20], OperandSize::Qword)
}

fn vfmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 45, 212, 168, 41], OperandSize::Qword)
}

