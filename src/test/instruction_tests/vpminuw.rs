use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 58, 216], OperandSize::Dword)
}

fn vpminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1449262586, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 58, 44, 205, 250, 253, 97, 86], OperandSize::Dword)
}

fn vpminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 58, 245], OperandSize::Qword)
}

fn vpminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 58, 4, 65], OperandSize::Qword)
}

fn vpminuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 58, 203], OperandSize::Dword)
}

fn vpminuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 58, 4, 192], OperandSize::Dword)
}

fn vpminuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 58, 235], OperandSize::Qword)
}

fn vpminuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 794055160, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 58, 12, 117, 248, 81, 84, 47], OperandSize::Qword)
}

fn vpminuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 143, 58, 206], OperandSize::Dword)
}

fn vpminuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 58, 4, 206], OperandSize::Dword)
}

fn vpminuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 29, 129, 58, 230], OperandSize::Qword)
}

fn vpminuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 58, 12, 202], OperandSize::Qword)
}

fn vpminuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 170, 58, 247], OperandSize::Dword)
}

fn vpminuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 317419065, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 58, 28, 157, 57, 110, 235, 18], OperandSize::Dword)
}

fn vpminuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 77, 175, 58, 206], OperandSize::Qword)
}

fn vpminuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1719256667, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 164, 58, 148, 134, 91, 198, 121, 102], OperandSize::Qword)
}

fn vpminuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 58, 192], OperandSize::Dword)
}

fn vpminuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 58, 20, 79], OperandSize::Dword)
}

fn vpminuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 37, 196, 58, 253], OperandSize::Qword)
}

fn vpminuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1096725718, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 13, 206, 58, 188, 214, 214, 180, 94, 65], OperandSize::Qword)
}

