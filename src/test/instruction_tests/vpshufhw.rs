use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 208, 114], OperandSize::Dword)
}

fn vpshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 20, 94, 71], OperandSize::Dword)
}

fn vpshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 230, 46], OperandSize::Qword)
}

fn vpshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RAX, 488433351, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 168, 199, 230, 28, 29, 15], OperandSize::Qword)
}

fn vpshufhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 206, 16], OperandSize::Dword)
}

fn vpshufhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 729142741, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 172, 206, 213, 213, 117, 43, 37], OperandSize::Dword)
}

fn vpshufhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 236, 79], OperandSize::Qword)
}

fn vpshufhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 4, 240, 26], OperandSize::Qword)
}

fn vpshufhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 112, 205, 67], OperandSize::Dword)
}

fn vpshufhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 455217688, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 112, 28, 125, 24, 18, 34, 27, 57], OperandSize::Dword)
}

fn vpshufhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 137, 112, 227, 41], OperandSize::Qword)
}

fn vpshufhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 139532361, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 142, 112, 188, 145, 73, 24, 81, 8, 58], OperandSize::Qword)
}

fn vpshufhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 112, 234, 75], OperandSize::Dword)
}

fn vpshufhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 2111185581, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 112, 188, 218, 173, 34, 214, 125, 69], OperandSize::Dword)
}

fn vpshufhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM20)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 126, 175, 112, 212, 102], OperandSize::Qword)
}

fn vpshufhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM20)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 174, 112, 35, 61], OperandSize::Qword)
}

fn vpshufhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 112, 200, 86], OperandSize::Dword)
}

fn vpshufhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1050949097, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 112, 12, 253, 233, 53, 164, 62, 32], OperandSize::Dword)
}

fn vpshufhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 126, 205, 112, 237, 70], OperandSize::Qword)
}

fn vpshufhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 335270654, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 202, 112, 52, 205, 254, 210, 251, 19, 118], OperandSize::Qword)
}

