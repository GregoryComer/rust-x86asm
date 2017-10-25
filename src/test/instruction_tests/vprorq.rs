use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 114, 198, 58], OperandSize::Dword)
}

fn vprorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1866845232, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 114, 132, 128, 48, 204, 69, 111, 57], OperandSize::Dword)
}

fn vprorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1343431454, Some(OperandSize::Qword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 157, 114, 4, 77, 30, 35, 19, 80, 77], OperandSize::Dword)
}

fn vprorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM17)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 197, 141, 114, 193, 33], OperandSize::Qword)
}

fn vprorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1945037681, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 143, 114, 4, 125, 113, 235, 238, 115, 115], OperandSize::Qword)
}

fn vprorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 860193858, Some(OperandSize::Qword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 141, 146, 114, 4, 69, 66, 132, 69, 51, 91], OperandSize::Qword)
}

fn vprorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 114, 196, 40], OperandSize::Dword)
}

fn vprorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 174, 114, 4, 150, 99], OperandSize::Dword)
}

fn vprorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1094147832, Some(OperandSize::Qword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 185, 114, 4, 149, 248, 94, 55, 65, 35], OperandSize::Dword)
}

fn vprorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM16)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 205, 172, 114, 192, 76], OperandSize::Qword)
}

fn vprorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM21)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 167, 114, 1, 104], OperandSize::Qword)
}

fn vprorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 177, 114, 4, 211, 41], OperandSize::Qword)
}

fn vprorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 207, 114, 198, 51], OperandSize::Dword)
}

fn vprorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 204, 114, 7, 61], OperandSize::Dword)
}

fn vprorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 154906258, Some(OperandSize::Qword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 217, 114, 132, 89, 146, 174, 59, 9, 110], OperandSize::Dword)
}

fn vprorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM29)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 229, 206, 114, 197, 12], OperandSize::Qword)
}

fn vprorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 279682721, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 199, 114, 4, 181, 161, 158, 171, 16, 44], OperandSize::Qword)
}

fn vprorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 884630149, Some(OperandSize::Qword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 181, 221, 114, 4, 117, 133, 98, 186, 52, 19], OperandSize::Qword)
}

