use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprolq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 114, 207, 77], OperandSize::Dword)
}

fn vprolq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 141, 114, 12, 187, 115], OperandSize::Dword)
}

fn vprolq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EBX, 1188800985, Some(OperandSize::Qword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 159, 114, 139, 217, 169, 219, 70, 15], OperandSize::Dword)
}

fn vprolq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM23)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 197, 137, 114, 207, 94], OperandSize::Qword)
}

fn vprolq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 11655580, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 139, 114, 12, 125, 156, 217, 177, 0, 4], OperandSize::Qword)
}

fn vprolq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM18)), operand2: Some(IndirectDisplaced(RDI, 1348127978, Some(OperandSize::Qword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 147, 114, 143, 234, 204, 90, 80, 100], OperandSize::Qword)
}

fn vprolq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 170, 114, 202, 40], OperandSize::Dword)
}

fn vprolq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1373112214, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 171, 114, 12, 189, 150, 7, 216, 81, 33], OperandSize::Dword)
}

fn vprolq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 318242600, Some(OperandSize::Qword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 189, 114, 140, 214, 40, 255, 247, 18, 76], OperandSize::Dword)
}

fn vprolq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM23)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 237, 171, 114, 207, 73], OperandSize::Qword)
}

fn vprolq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1894306006, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 172, 114, 140, 215, 214, 208, 232, 112, 90], OperandSize::Qword)
}

fn vprolq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 165, 191, 114, 12, 187, 39], OperandSize::Qword)
}

fn vprolq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 201, 114, 200, 25], OperandSize::Dword)
}

fn vprolq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1681154442, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 202, 114, 140, 154, 138, 97, 52, 100, 52], OperandSize::Dword)
}

fn vprolq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EAX, 1796635729, Some(OperandSize::Qword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 223, 114, 136, 81, 124, 22, 107, 120], OperandSize::Dword)
}

fn vprolq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM22)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 189, 206, 114, 206, 9], OperandSize::Qword)
}

fn vprolq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(RSI, 148564363, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 207, 114, 142, 139, 233, 218, 8, 5], OperandSize::Qword)
}

fn vprolq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 620623463, Some(OperandSize::Qword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 165, 210, 114, 12, 205, 103, 246, 253, 36, 63], OperandSize::Qword)
}

