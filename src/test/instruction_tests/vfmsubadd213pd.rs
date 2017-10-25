use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsubadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 167, 236], OperandSize::Dword)
}

fn vfmsubadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1035375596, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 167, 156, 242, 236, 147, 182, 61], OperandSize::Dword)
}

fn vfmsubadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 167, 238], OperandSize::Qword)
}

fn vfmsubadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 167, 52, 73], OperandSize::Qword)
}

fn vfmsubadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 167, 240], OperandSize::Dword)
}

fn vfmsubadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ESI, 1591691256, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 167, 142, 248, 71, 223, 94], OperandSize::Dword)
}

fn vfmsubadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 167, 252], OperandSize::Qword)
}

fn vfmsubadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 167, 22], OperandSize::Qword)
}

fn vfmsubadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 167, 238], OperandSize::Dword)
}

fn vfmsubadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 167, 60, 185], OperandSize::Dword)
}

fn vfmsubadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 154, 167, 42], OperandSize::Dword)
}

fn vfmsubadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 149, 137, 167, 228], OperandSize::Qword)
}

fn vfmsubadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 245, 135, 167, 60, 247], OperandSize::Qword)
}

fn vfmsubadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 141, 149, 167, 7], OperandSize::Qword)
}

fn vfmsubadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 167, 231], OperandSize::Dword)
}

fn vfmsubadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1369311987, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 170, 167, 4, 197, 243, 10, 158, 81], OperandSize::Dword)
}

fn vfmsubadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1381353799, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 186, 167, 188, 184, 71, 201, 85, 82], OperandSize::Dword)
}

fn vfmsubadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 213, 172, 167, 241], OperandSize::Qword)
}

fn vfmsubadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 189, 167, 167, 4, 122], OperandSize::Qword)
}

fn vfmsubadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RSI, 1986207281, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 157, 189, 167, 174, 49, 30, 99, 118], OperandSize::Qword)
}

fn vfmsubadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 188, 167, 192], OperandSize::Dword)
}

fn vfmsubadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 203, 167, 4, 176], OperandSize::Dword)
}

fn vfmsubadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 80721209, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 218, 167, 12, 181, 57, 181, 207, 4], OperandSize::Dword)
}

fn vfmsubadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 213, 178, 167, 225], OperandSize::Qword)
}

fn vfmsubadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RDX, 711271233, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 149, 195, 167, 186, 65, 35, 101, 42], OperandSize::Qword)
}

fn vfmsubadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectDisplaced(RDX, 198856348, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 141, 217, 167, 162, 156, 78, 218, 11], OperandSize::Qword)
}

