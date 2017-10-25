use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 170, 206], OperandSize::Dword)
}

fn vfmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1680746324, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 170, 148, 115, 84, 39, 46, 100], OperandSize::Dword)
}

fn vfmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 170, 213], OperandSize::Qword)
}

fn vfmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 681196803, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 170, 28, 149, 3, 61, 154, 40], OperandSize::Qword)
}

fn vfmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 170, 222], OperandSize::Dword)
}

fn vfmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 170, 4, 126], OperandSize::Dword)
}

fn vfmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 170, 208], OperandSize::Qword)
}

fn vfmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 1457506198, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 170, 150, 150, 199, 223, 86], OperandSize::Qword)
}

fn vfmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 170, 212], OperandSize::Dword)
}

fn vfmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 170, 7], OperandSize::Dword)
}

fn vfmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 264886130, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 170, 188, 208, 114, 215, 201, 15], OperandSize::Dword)
}

fn vfmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 245, 138, 170, 231], OperandSize::Qword)
}

fn vfmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 1647014742, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 229, 141, 170, 147, 86, 115, 43, 98], OperandSize::Qword)
}

fn vfmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 157, 157, 170, 4, 248], OperandSize::Qword)
}

fn vfmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 170, 197], OperandSize::Dword)
}

fn vfmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 170, 8], OperandSize::Dword)
}

fn vfmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 1228690776, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 189, 170, 171, 88, 85, 60, 73], OperandSize::Dword)
}

fn vfmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 213, 173, 170, 250], OperandSize::Qword)
}

fn vfmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 426556513, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 133, 162, 170, 44, 221, 97, 188, 108, 25], OperandSize::Qword)
}

fn vfmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RCX, 359998206, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 177, 170, 153, 254, 34, 117, 21], OperandSize::Qword)
}

fn vfmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 157, 170, 193], OperandSize::Dword)
}

fn vfmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 170, 36, 127], OperandSize::Dword)
}

fn vfmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 221, 170, 60, 81], OperandSize::Dword)
}

fn vfmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 237, 255, 170, 192], OperandSize::Qword)
}

fn vfmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RCX, 2060357225, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 204, 170, 185, 105, 142, 206, 122], OperandSize::Qword)
}

fn vfmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 688262413, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 181, 210, 170, 140, 90, 13, 13, 6, 41], OperandSize::Qword)
}

