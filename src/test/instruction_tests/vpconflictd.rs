use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpconflictd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 196, 249], OperandSize::Dword)
}

fn vpconflictd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 196, 40], OperandSize::Dword)
}

fn vpconflictd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ECX, 1679648127, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 196, 137, 127, 101, 29, 100], OperandSize::Dword)
}

fn vpconflictd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 125, 137, 196, 241], OperandSize::Qword)
}

fn vpconflictd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 84198744, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 196, 151, 88, 197, 4, 5], OperandSize::Qword)
}

fn vpconflictd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 2014254947, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 125, 154, 196, 180, 127, 99, 23, 15, 120], OperandSize::Qword)
}

fn vpconflictd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 196, 211], OperandSize::Dword)
}

fn vpconflictd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1255986192, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 196, 52, 213, 16, 212, 220, 74], OperandSize::Dword)
}

fn vpconflictd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 709024621, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 196, 156, 127, 109, 219, 66, 42], OperandSize::Dword)
}

fn vpconflictd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 125, 170, 196, 252], OperandSize::Qword)
}

fn vpconflictd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 170, 196, 4, 121], OperandSize::Qword)
}

fn vpconflictd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1196786291, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 190, 196, 4, 221, 115, 130, 85, 71], OperandSize::Qword)
}

fn vpconflictd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 196, 246], OperandSize::Dword)
}

fn vpconflictd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 456182014, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 196, 180, 240, 254, 200, 48, 27], OperandSize::Dword)
}

fn vpconflictd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1824134425, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 196, 156, 243, 25, 21, 186, 108], OperandSize::Dword)
}

fn vpconflictd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 206, 196, 200], OperandSize::Qword)
}

fn vpconflictd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 196, 44, 223], OperandSize::Qword)
}

fn vpconflictd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 2147006196, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 196, 164, 177, 244, 182, 248, 127], OperandSize::Qword)
}

