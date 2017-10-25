use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 60, 202], OperandSize::Dword)
}

fn vpmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 721647789, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 60, 12, 181, 173, 120, 3, 43], OperandSize::Dword)
}

fn vpmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 60, 252], OperandSize::Qword)
}

fn vpmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 364685052, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 60, 130, 252, 166, 188, 21], OperandSize::Qword)
}

fn vpmaxsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 60, 246], OperandSize::Dword)
}

fn vpmaxsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 518631396, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 60, 28, 93, 228, 175, 233, 30], OperandSize::Dword)
}

fn vpmaxsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 60, 199], OperandSize::Qword)
}

fn vpmaxsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 92859329, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 60, 172, 218, 193, 235, 136, 5], OperandSize::Qword)
}

fn vpmaxsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 143, 60, 232], OperandSize::Dword)
}

fn vpmaxsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 142, 60, 44, 130], OperandSize::Dword)
}

fn vpmaxsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 109, 141, 60, 220], OperandSize::Qword)
}

fn vpmaxsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 131, 60, 36, 201], OperandSize::Qword)
}

fn vpmaxsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 172, 60, 194], OperandSize::Dword)
}

fn vpmaxsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 60, 56], OperandSize::Dword)
}

fn vpmaxsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 37, 166, 60, 209], OperandSize::Qword)
}

fn vpmaxsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 29, 172, 60, 31], OperandSize::Qword)
}

fn vpmaxsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 202, 60, 220], OperandSize::Dword)
}

fn vpmaxsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1485829031, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 201, 60, 52, 125, 167, 243, 143, 88], OperandSize::Dword)
}

fn vpmaxsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 69, 202, 60, 193], OperandSize::Qword)
}

fn vpmaxsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 75161429, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 93, 198, 60, 20, 93, 85, 223, 122, 4], OperandSize::Qword)
}

