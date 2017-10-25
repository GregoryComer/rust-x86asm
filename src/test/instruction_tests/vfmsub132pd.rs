use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 154, 203], OperandSize::Dword)
}

fn vfmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1646400298, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 154, 148, 122, 42, 19, 34, 98], OperandSize::Dword)
}

fn vfmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 154, 231], OperandSize::Qword)
}

fn vfmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 154, 44, 95], OperandSize::Qword)
}

fn vfmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 154, 235], OperandSize::Dword)
}

fn vfmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 308378215, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 154, 132, 211, 103, 122, 97, 18], OperandSize::Dword)
}

fn vfmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 154, 242], OperandSize::Qword)
}

fn vfmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 2101676639, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 154, 164, 90, 95, 10, 69, 125], OperandSize::Qword)
}

fn vfmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 140, 154, 202], OperandSize::Dword)
}

fn vfmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1504187335, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 143, 154, 164, 195, 199, 19, 168, 89], OperandSize::Dword)
}

fn vfmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 154, 36, 176], OperandSize::Dword)
}

fn vfmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 245, 139, 154, 213], OperandSize::Qword)
}

fn vfmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 149, 143, 154, 24], OperandSize::Qword)
}

fn vfmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RSI, 1524514811, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 237, 150, 154, 150, 251, 63, 222, 90], OperandSize::Qword)
}

fn vfmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 154, 251], OperandSize::Dword)
}

fn vfmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 569942213, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 154, 188, 198, 197, 160, 248, 33], OperandSize::Dword)
}

fn vfmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 187, 154, 60, 251], OperandSize::Dword)
}

fn vfmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 253, 170, 154, 254], OperandSize::Qword)
}

fn vfmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 955815952, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 161, 154, 52, 245, 16, 152, 248, 56], OperandSize::Qword)
}

fn vfmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 213, 188, 154, 44, 195], OperandSize::Qword)
}

fn vfmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 191, 154, 244], OperandSize::Dword)
}

fn vfmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 485616073, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 205, 154, 148, 82, 201, 233, 241, 28], OperandSize::Dword)
}

fn vfmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 222, 154, 32], OperandSize::Dword)
}

fn vfmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 157, 149, 154, 204], OperandSize::Qword)
}

fn vfmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 197, 193, 154, 23], OperandSize::Qword)
}

fn vfmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 229, 223, 154, 15], OperandSize::Qword)
}

