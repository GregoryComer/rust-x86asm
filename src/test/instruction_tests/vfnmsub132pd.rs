use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 158, 204], OperandSize::Dword)
}

fn vfnmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1000686025, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 158, 36, 69, 201, 65, 165, 59], OperandSize::Dword)
}

fn vfnmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 158, 231], OperandSize::Qword)
}

fn vfnmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 158, 17], OperandSize::Qword)
}

fn vfnmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 158, 231], OperandSize::Dword)
}

fn vfnmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 989941176, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 158, 12, 253, 184, 77, 1, 59], OperandSize::Dword)
}

fn vfnmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 158, 235], OperandSize::Qword)
}

fn vfnmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1926957919, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 158, 188, 66, 95, 11, 219, 114], OperandSize::Qword)
}

fn vfnmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 158, 198], OperandSize::Dword)
}

fn vfnmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 76197801, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 158, 178, 169, 175, 138, 4], OperandSize::Dword)
}

fn vfnmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 1187074871, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 155, 158, 162, 55, 83, 193, 70], OperandSize::Dword)
}

fn vfnmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 165, 134, 158, 253], OperandSize::Qword)
}

fn vfnmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 213, 142, 158, 0], OperandSize::Qword)
}

fn vfnmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RSI, 1760403346, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 149, 154, 158, 142, 146, 159, 237, 104], OperandSize::Qword)
}

fn vfnmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 158, 251], OperandSize::Dword)
}

fn vfnmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 158, 1], OperandSize::Dword)
}

fn vfnmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 107816937, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 186, 158, 148, 64, 233, 39, 109, 6], OperandSize::Dword)
}

fn vfnmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 165, 169, 158, 221], OperandSize::Qword)
}

fn vfnmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 166, 158, 4, 113], OperandSize::Qword)
}

fn vfnmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 165, 191, 158, 4, 243], OperandSize::Qword)
}

fn vfnmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 250, 158, 199], OperandSize::Dword)
}

fn vfnmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 158, 51], OperandSize::Dword)
}

fn vfnmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 433918817, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 158, 12, 125, 97, 19, 221, 25], OperandSize::Dword)
}

fn vfnmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 149, 245, 158, 212], OperandSize::Qword)
}

fn vfnmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 62529730, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 141, 204, 158, 28, 141, 194, 32, 186, 3], OperandSize::Qword)
}

fn vfnmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RSI, 1906879761, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 181, 219, 158, 190, 17, 173, 168, 113], OperandSize::Qword)
}

