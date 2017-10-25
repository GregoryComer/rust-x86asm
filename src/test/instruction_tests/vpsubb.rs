use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 248, 254], OperandSize::Dword)
}

fn vpsubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 608967750, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 248, 156, 119, 70, 28, 76, 36], OperandSize::Dword)
}

fn vpsubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 248, 198], OperandSize::Qword)
}

fn vpsubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 248, 28, 191], OperandSize::Qword)
}

fn vpsubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 248, 211], OperandSize::Dword)
}

fn vpsubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 248, 12, 114], OperandSize::Dword)
}

fn vpsubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 248, 253], OperandSize::Qword)
}

fn vpsubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 389842794, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 248, 130, 106, 135, 60, 23], OperandSize::Qword)
}

fn vpsubb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 248, 242], OperandSize::Dword)
}

fn vpsubb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1493310625, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 248, 60, 221, 161, 28, 2, 89], OperandSize::Dword)
}

fn vpsubb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 117, 137, 248, 250], OperandSize::Qword)
}

fn vpsubb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 69, 139, 248, 44, 95], OperandSize::Qword)
}

fn vpsubb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 248, 238], OperandSize::Dword)
}

fn vpsubb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 248, 57], OperandSize::Dword)
}

fn vpsubb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 21, 164, 248, 226], OperandSize::Qword)
}

fn vpsubb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 117, 166, 248, 51], OperandSize::Qword)
}

fn vpsubb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 206, 248, 196], OperandSize::Dword)
}

fn vpsubb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 248, 12, 178], OperandSize::Dword)
}

fn vpsubb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 37, 195, 248, 216], OperandSize::Qword)
}

fn vpsubb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1347548007, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 93, 202, 248, 180, 139, 103, 243, 81, 80], OperandSize::Qword)
}

