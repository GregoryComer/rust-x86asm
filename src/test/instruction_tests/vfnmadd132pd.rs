use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 156, 226], OperandSize::Dword)
}

fn vfnmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 156, 60, 223], OperandSize::Dword)
}

fn vfnmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 156, 248], OperandSize::Qword)
}

fn vfnmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 156, 52, 123], OperandSize::Qword)
}

fn vfnmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 197], OperandSize::Dword)
}

fn vfnmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 156, 44, 135], OperandSize::Dword)
}

fn vfnmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 255], OperandSize::Qword)
}

fn vfnmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 634153268, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 172, 151, 52, 105, 204, 37], OperandSize::Qword)
}

fn vfnmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 141, 156, 247], OperandSize::Dword)
}

fn vfnmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 156, 16], OperandSize::Dword)
}

fn vfnmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 158, 156, 8], OperandSize::Dword)
}

fn vfnmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 173, 133, 156, 253], OperandSize::Qword)
}

fn vfnmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 129, 156, 36, 199], OperandSize::Qword)
}

fn vfnmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RSI, 456793616, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 173, 155, 156, 166, 16, 30, 58, 27], OperandSize::Qword)
}

fn vfnmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 156, 224], OperandSize::Dword)
}

fn vfnmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 156, 26], OperandSize::Dword)
}

fn vfnmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 188, 156, 42], OperandSize::Dword)
}

fn vfnmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 133, 175, 156, 231], OperandSize::Qword)
}

fn vfnmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RDI, 1309131340, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 181, 169, 156, 167, 76, 194, 7, 78], OperandSize::Qword)
}

fn vfnmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 141, 186, 156, 43], OperandSize::Qword)
}

fn vfnmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 219, 156, 227], OperandSize::Dword)
}

fn vfnmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 156, 52, 250], OperandSize::Dword)
}

fn vfnmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1953672915, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 222, 156, 52, 157, 211, 174, 114, 116], OperandSize::Dword)
}

fn vfnmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 165, 254, 156, 248], OperandSize::Qword)
}

fn vfnmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1957928873, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 197, 156, 148, 215, 169, 159, 179, 116], OperandSize::Qword)
}

fn vfnmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1678082631, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 245, 214, 156, 44, 77, 71, 130, 5, 100], OperandSize::Qword)
}

