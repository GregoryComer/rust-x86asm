use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtps2ph_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 193, 9], OperandSize::Dword)
}

fn vcvtps2ph_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 450348790, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 52, 253, 246, 198, 215, 26, 17], OperandSize::Dword)
}

fn vcvtps2ph_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 238, 14], OperandSize::Qword)
}

fn vcvtps2ph_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1261858467, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 44, 253, 163, 110, 54, 75, 107], OperandSize::Qword)
}

fn vcvtps2ph_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 217, 126], OperandSize::Dword)
}

fn vcvtps2ph_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(EAX, 708136721, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 184, 17, 79, 53, 42, 17], OperandSize::Dword)
}

fn vcvtps2ph_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 234, 20], OperandSize::Qword)
}

fn vcvtps2ph_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1769402171, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 60, 149, 59, 239, 118, 105, 79], OperandSize::Qword)
}

fn vcvtps2ph_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 29, 220, 38], OperandSize::Dword)
}

fn vcvtps2ph_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 7, 56], OperandSize::Dword)
}

fn vcvtps2ph_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM25)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 35, 125, 141, 29, 203, 94], OperandSize::Qword)
}

fn vcvtps2ph_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1207461982, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM17)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 125, 8, 29, 140, 74, 94, 104, 248, 71, 61], OperandSize::Qword)
}

fn vcvtps2ph_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 169, 29, 231, 54], OperandSize::Dword)
}

fn vcvtps2ph_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 970042596, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 172, 135, 228, 172, 209, 57, 98], OperandSize::Dword)
}

fn vcvtps2ph_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM24)), operand2: Some(Direct(YMM22)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 131, 125, 170, 29, 240, 97], OperandSize::Qword)
}

fn vcvtps2ph_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 502305431, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM22)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 125, 40, 29, 52, 213, 151, 146, 240, 29, 120], OperandSize::Qword)
}

fn vcvtps2ph_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 156, 29, 249, 103], OperandSize::Dword)
}

fn vcvtps2ph_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 29, 50, 43], OperandSize::Dword)
}

fn vcvtps2ph_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM24)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 125, 159, 29, 248, 38], OperandSize::Qword)
}

fn vcvtps2ph_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(RCX, 1560203981, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM28)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 72, 29, 161, 205, 210, 254, 92, 84], OperandSize::Qword)
}

