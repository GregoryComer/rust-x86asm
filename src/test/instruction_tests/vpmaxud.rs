use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 63, 222], OperandSize::Dword)
}

fn vpmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 63, 10], OperandSize::Dword)
}

fn vpmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 63, 234], OperandSize::Qword)
}

fn vpmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1058157291, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 63, 12, 205, 235, 50, 18, 63], OperandSize::Qword)
}

fn vpmaxud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 63, 236], OperandSize::Dword)
}

fn vpmaxud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 63, 12, 216], OperandSize::Dword)
}

fn vpmaxud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 63, 242], OperandSize::Qword)
}

fn vpmaxud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 63, 12, 200], OperandSize::Qword)
}

fn vpmaxud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 137, 63, 207], OperandSize::Dword)
}

fn vpmaxud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 154284420, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 63, 177, 132, 49, 50, 9], OperandSize::Dword)
}

fn vpmaxud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1581120430, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 157, 63, 140, 79, 174, 251, 61, 94], OperandSize::Dword)
}

fn vpmaxud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 101, 137, 63, 198], OperandSize::Qword)
}

fn vpmaxud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 134, 63, 17], OperandSize::Qword)
}

fn vpmaxud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 69, 147, 63, 60, 118], OperandSize::Qword)
}

fn vpmaxud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 63, 195], OperandSize::Dword)
}

fn vpmaxud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 172, 63, 51], OperandSize::Dword)
}

fn vpmaxud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 186, 63, 44, 193], OperandSize::Dword)
}

fn vpmaxud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 61, 173, 63, 215], OperandSize::Qword)
}

fn vpmaxud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 678556622, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 77, 165, 63, 188, 119, 206, 243, 113, 40], OperandSize::Qword)
}

fn vpmaxud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 160079082, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 117, 186, 63, 148, 247, 234, 156, 138, 9], OperandSize::Qword)
}

fn vpmaxud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 205, 63, 198], OperandSize::Dword)
}

fn vpmaxud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 184057720, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 205, 63, 135, 120, 127, 248, 10], OperandSize::Dword)
}

fn vpmaxud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 128738557, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 63, 180, 74, 253, 100, 172, 7], OperandSize::Dword)
}

fn vpmaxud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 5, 202, 63, 210], OperandSize::Qword)
}

fn vpmaxud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 13, 202, 63, 54], OperandSize::Qword)
}

fn vpmaxud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RDI, 330784684, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 63, 159, 172, 95, 183, 19], OperandSize::Qword)
}

