use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 184, 243], OperandSize::Dword)
}

fn vfmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 184, 48], OperandSize::Dword)
}

fn vfmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 184, 211], OperandSize::Qword)
}

fn vfmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1117938549, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 184, 20, 69, 117, 99, 162, 66], OperandSize::Qword)
}

fn vfmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 184, 228], OperandSize::Dword)
}

fn vfmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 98458103, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 184, 60, 69, 247, 89, 222, 5], OperandSize::Dword)
}

fn vfmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 184, 254], OperandSize::Qword)
}

fn vfmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 184, 60, 121], OperandSize::Qword)
}

fn vfmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 184, 252], OperandSize::Dword)
}

fn vfmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 184, 7], OperandSize::Dword)
}

fn vfmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 970966828, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 153, 184, 20, 197, 44, 199, 223, 57], OperandSize::Dword)
}

fn vfmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 237, 130, 184, 254], OperandSize::Qword)
}

fn vfmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 213556691, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 135, 184, 156, 203, 211, 157, 186, 12], OperandSize::Qword)
}

fn vfmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 44588817, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 245, 148, 184, 164, 146, 17, 95, 168, 2], OperandSize::Qword)
}

fn vfmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 171, 184, 231], OperandSize::Dword)
}

fn vfmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1969235741, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 171, 184, 148, 131, 29, 39, 96, 117], OperandSize::Dword)
}

fn vfmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 1663683482, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 191, 184, 137, 154, 203, 41, 99], OperandSize::Dword)
}

fn vfmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 229, 172, 184, 227], OperandSize::Qword)
}

fn vfmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RAX, 1471571251, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 141, 163, 184, 168, 51, 101, 182, 87], OperandSize::Qword)
}

fn vfmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 157, 188, 184, 57], OperandSize::Qword)
}

fn vfmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 190, 184, 238], OperandSize::Dword)
}

fn vfmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 202, 184, 55], OperandSize::Dword)
}

fn vfmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1423251360, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 220, 184, 60, 245, 160, 23, 213, 84], OperandSize::Dword)
}

fn vfmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 229, 177, 184, 208], OperandSize::Qword)
}

fn vfmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 205, 207, 184, 48], OperandSize::Qword)
}

fn vfmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1972462424, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 133, 211, 184, 180, 154, 88, 99, 145, 117], OperandSize::Qword)
}

