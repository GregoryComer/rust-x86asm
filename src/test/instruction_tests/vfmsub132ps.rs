use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 154, 209], OperandSize::Dword)
}

fn vfmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 154, 34], OperandSize::Dword)
}

fn vfmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 154, 211], OperandSize::Qword)
}

fn vfmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 154, 12, 250], OperandSize::Qword)
}

fn vfmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 154, 205], OperandSize::Dword)
}

fn vfmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 154, 44, 151], OperandSize::Dword)
}

fn vfmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 154, 227], OperandSize::Qword)
}

fn vfmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 154, 4, 119], OperandSize::Qword)
}

fn vfmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 154, 212], OperandSize::Dword)
}

fn vfmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 138, 154, 34], OperandSize::Dword)
}

fn vfmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 957856376, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 158, 154, 60, 141, 120, 186, 23, 57], OperandSize::Dword)
}

fn vfmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 109, 141, 154, 254], OperandSize::Qword)
}

fn vfmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 134, 154, 52, 80], OperandSize::Qword)
}

fn vfmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1264556637, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 5, 159, 154, 188, 195, 93, 154, 95, 75], OperandSize::Qword)
}

fn vfmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 154, 250], OperandSize::Dword)
}

fn vfmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 154, 52, 64], OperandSize::Dword)
}

fn vfmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 189, 154, 28, 209], OperandSize::Dword)
}

fn vfmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 13, 165, 154, 201], OperandSize::Qword)
}

fn vfmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1186249437, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 5, 165, 154, 164, 146, 221, 186, 180, 70], OperandSize::Qword)
}

fn vfmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 854022118, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 109, 186, 154, 151, 230, 87, 231, 50], OperandSize::Qword)
}

fn vfmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 220, 154, 207], OperandSize::Dword)
}

fn vfmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2112379035, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 206, 154, 36, 77, 155, 88, 232, 125], OperandSize::Dword)
}

fn vfmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 1662862831, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 222, 154, 129, 239, 69, 29, 99], OperandSize::Dword)
}

fn vfmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 77, 221, 154, 220], OperandSize::Qword)
}

fn vfmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1895395962, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 37, 204, 154, 60, 69, 122, 114, 249, 112], OperandSize::Qword)
}

fn vfmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 218, 154, 51], OperandSize::Qword)
}

