use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 62, 218], OperandSize::Dword)
}

#[test]
fn vpmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 62, 60, 65], OperandSize::Dword)
}

#[test]
fn vpmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 62, 247], OperandSize::Qword)
}

#[test]
fn vpmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 62, 4, 153], OperandSize::Qword)
}

#[test]
fn vpmaxuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 62, 215], OperandSize::Dword)
}

#[test]
fn vpmaxuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 146542146, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 62, 36, 93, 66, 14, 188, 8], OperandSize::Dword)
}

#[test]
fn vpmaxuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 62, 220], OperandSize::Qword)
}

#[test]
fn vpmaxuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1633348272, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 62, 148, 242, 176, 234, 90, 97], OperandSize::Qword)
}

#[test]
fn vpmaxuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 62, 236], OperandSize::Dword)
}

#[test]
fn vpmaxuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1667199921, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 62, 20, 77, 177, 115, 95, 99], OperandSize::Dword)
}

#[test]
fn vpmaxuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 21, 129, 62, 201], OperandSize::Qword)
}

#[test]
fn vpmaxuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 93, 140, 62, 56], OperandSize::Qword)
}

#[test]
fn vpmaxuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 62, 201], OperandSize::Dword)
}

#[test]
fn vpmaxuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1938901388, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 62, 185, 140, 73, 145, 115], OperandSize::Dword)
}

#[test]
fn vpmaxuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 69, 170, 62, 202], OperandSize::Qword)
}

#[test]
fn vpmaxuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 537498927, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 29, 166, 62, 140, 247, 47, 149, 9, 32], OperandSize::Qword)
}

#[test]
fn vpmaxuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 201, 62, 192], OperandSize::Dword)
}

#[test]
fn vpmaxuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 62, 12, 190], OperandSize::Dword)
}

#[test]
fn vpmaxuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 125, 193, 62, 192], OperandSize::Qword)
}

#[test]
fn vpmaxuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 45, 201, 62, 14], OperandSize::Qword)
}

