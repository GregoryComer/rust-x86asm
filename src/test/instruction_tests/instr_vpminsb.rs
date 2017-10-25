use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 56, 221], OperandSize::Dword)
}

#[test]
fn vpminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 161955095, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 56, 188, 95, 23, 61, 167, 9], OperandSize::Dword)
}

#[test]
fn vpminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 56, 195], OperandSize::Qword)
}

#[test]
fn vpminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1091307482, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 56, 4, 141, 218, 7, 12, 65], OperandSize::Qword)
}

#[test]
fn vpminsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 56, 235], OperandSize::Dword)
}

#[test]
fn vpminsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 796918846, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 56, 44, 93, 62, 4, 128, 47], OperandSize::Dword)
}

#[test]
fn vpminsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 56, 229], OperandSize::Qword)
}

#[test]
fn vpminsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RAX, 435633001, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 56, 184, 105, 59, 247, 25], OperandSize::Qword)
}

#[test]
fn vpminsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 56, 218], OperandSize::Dword)
}

#[test]
fn vpminsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 2099541198, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 141, 56, 156, 113, 206, 116, 36, 125], OperandSize::Dword)
}

#[test]
fn vpminsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 56, 248], OperandSize::Qword)
}

#[test]
fn vpminsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 69, 139, 56, 12, 87], OperandSize::Qword)
}

#[test]
fn vpminsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 56, 214], OperandSize::Dword)
}

#[test]
fn vpminsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 56, 36, 240], OperandSize::Dword)
}

#[test]
fn vpminsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 21, 162, 56, 215], OperandSize::Qword)
}

#[test]
fn vpminsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 674640056, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 101, 167, 56, 132, 177, 184, 48, 54, 40], OperandSize::Qword)
}

#[test]
fn vpminsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 207, 56, 246], OperandSize::Dword)
}

#[test]
fn vpminsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 358911461, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 56, 52, 77, 229, 141, 100, 21], OperandSize::Dword)
}

#[test]
fn vpminsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 117, 206, 56, 222], OperandSize::Qword)
}

#[test]
fn vpminsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 642319871, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 77, 205, 56, 52, 181, 255, 5, 73, 38], OperandSize::Qword)
}

