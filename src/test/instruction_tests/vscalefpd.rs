use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vscalefpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 44, 219], OperandSize::Dword)
}

fn vscalefpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 44, 63], OperandSize::Dword)
}

fn vscalefpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 153, 44, 4, 75], OperandSize::Dword)
}

fn vscalefpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 237, 138, 44, 236], OperandSize::Qword)
}

fn vscalefpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RSI, 1263535351, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 44, 174, 247, 4, 80, 75], OperandSize::Qword)
}

fn vscalefpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 2033033538, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 221, 145, 44, 20, 253, 66, 161, 45, 121], OperandSize::Qword)
}

fn vscalefpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 44, 206], OperandSize::Dword)
}

fn vscalefpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 44, 12, 136], OperandSize::Dword)
}

fn vscalefpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 187, 44, 55], OperandSize::Dword)
}

fn vscalefpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 181, 163, 44, 211], OperandSize::Qword)
}

fn vscalefpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 165, 163, 44, 32], OperandSize::Qword)
}

fn vscalefpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RDI, 1008587368, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 177, 44, 143, 104, 210, 29, 60], OperandSize::Qword)
}

fn vscalefpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 187, 44, 214], OperandSize::Dword)
}

fn vscalefpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 207, 44, 46], OperandSize::Dword)
}

fn vscalefpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 1455469861, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 44, 154, 37, 181, 192, 86], OperandSize::Dword)
}

fn vscalefpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 133, 244, 44, 240], OperandSize::Qword)
}

fn vscalefpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 157, 204, 44, 11], OperandSize::Qword)
}

fn vscalefpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 480374542, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 245, 212, 44, 12, 213, 14, 239, 161, 28], OperandSize::Qword)
}

