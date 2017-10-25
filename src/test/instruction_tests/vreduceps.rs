use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vreduceps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 86, 230, 43], OperandSize::Dword)
}

fn vreduceps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 86, 55, 87], OperandSize::Dword)
}

fn vreduceps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 150479556, Some(OperandSize::Dword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 158, 86, 185, 196, 34, 248, 8, 50], OperandSize::Dword)
}

fn vreduceps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 125, 140, 86, 250, 26], OperandSize::Qword)
}

fn vreduceps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 125, 139, 86, 12, 219, 3], OperandSize::Qword)
}

fn vreduceps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM13)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 125, 159, 86, 42, 3], OperandSize::Qword)
}

fn vreduceps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 169, 86, 198, 118], OperandSize::Dword)
}

fn vreduceps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDX, 1188922373, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 172, 86, 178, 5, 132, 221, 70, 13], OperandSize::Dword)
}

fn vreduceps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 191, 86, 36, 83, 92], OperandSize::Dword)
}

fn vreduceps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM21)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 35, 125, 173, 86, 229, 99], OperandSize::Qword)
}

fn vreduceps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 630209192, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 125, 169, 86, 36, 197, 168, 58, 144, 37, 125], OperandSize::Qword)
}

fn vreduceps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM31)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 125, 188, 86, 63, 116], OperandSize::Qword)
}

fn vreduceps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 157, 86, 233, 88], OperandSize::Dword)
}

fn vreduceps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 204, 86, 62, 93], OperandSize::Dword)
}

fn vreduceps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ESI, 171376896, Some(OperandSize::Dword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 223, 86, 174, 0, 1, 55, 10, 73], OperandSize::Dword)
}

fn vreduceps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM31)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 125, 155, 86, 231, 97], OperandSize::Qword)
}

fn vreduceps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectDisplaced(RAX, 854463997, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 125, 206, 86, 168, 253, 21, 238, 50, 121], OperandSize::Qword)
}

fn vreduceps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1404252394, Some(OperandSize::Dword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 125, 220, 86, 148, 66, 234, 48, 179, 83, 56], OperandSize::Qword)
}

