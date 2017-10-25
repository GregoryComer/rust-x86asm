use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprolvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 21, 230], OperandSize::Dword)
}

fn vprolvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 870325033, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 21, 128, 41, 27, 224, 51], OperandSize::Dword)
}

fn vprolvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 21, 11], OperandSize::Dword)
}

fn vprolvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 13, 142, 21, 210], OperandSize::Qword)
}

fn vprolvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 13, 143, 21, 32], OperandSize::Qword)
}

fn vprolvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 1128827846, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 117, 159, 21, 175, 198, 139, 72, 67], OperandSize::Qword)
}

fn vprolvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 21, 226], OperandSize::Dword)
}

fn vprolvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 395847615, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 21, 4, 205, 191, 39, 152, 23], OperandSize::Dword)
}

fn vprolvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 191, 21, 20, 147], OperandSize::Dword)
}

fn vprolvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 93, 163, 21, 201], OperandSize::Qword)
}

fn vprolvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 37, 167, 21, 42], OperandSize::Qword)
}

fn vprolvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 85135770, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 69, 189, 21, 140, 190, 154, 17, 19, 5], OperandSize::Qword)
}

fn vprolvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 21, 244], OperandSize::Dword)
}

fn vprolvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 21, 35], OperandSize::Dword)
}

fn vprolvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 223, 21, 47], OperandSize::Dword)
}

fn vprolvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 109, 206, 21, 192], OperandSize::Qword)
}

fn vprolvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 21, 202, 21, 44, 241], OperandSize::Qword)
}

fn vprolvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 5, 212, 21, 59], OperandSize::Qword)
}

