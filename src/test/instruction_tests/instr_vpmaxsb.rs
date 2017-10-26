use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 60, 204], OperandSize::Dword)
}

#[test]
fn vpmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 60, 28, 176], OperandSize::Dword)
}

#[test]
fn vpmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 60, 228], OperandSize::Qword)
}

#[test]
fn vpmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 60, 62], OperandSize::Qword)
}

#[test]
fn vpmaxsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 60, 219], OperandSize::Dword)
}

#[test]
fn vpmaxsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 934976823, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 60, 20, 133, 55, 157, 186, 55], OperandSize::Dword)
}

#[test]
fn vpmaxsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 60, 227], OperandSize::Qword)
}

#[test]
fn vpmaxsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1434165818, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 60, 28, 221, 58, 162, 123, 85], OperandSize::Qword)
}

#[test]
fn vpmaxsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 60, 199], OperandSize::Dword)
}

#[test]
fn vpmaxsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 455046838, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 60, 12, 197, 182, 118, 31, 27], OperandSize::Dword)
}

#[test]
fn vpmaxsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 117, 138, 60, 251], OperandSize::Qword)
}

#[test]
fn vpmaxsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1595482567, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 77, 131, 60, 44, 213, 199, 33, 25, 95], OperandSize::Qword)
}

#[test]
fn vpmaxsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 60, 207], OperandSize::Dword)
}

#[test]
fn vpmaxsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 488538943, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 60, 12, 253, 63, 131, 30, 29], OperandSize::Dword)
}

#[test]
fn vpmaxsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 69, 170, 60, 206], OperandSize::Qword)
}

#[test]
fn vpmaxsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 85, 161, 60, 12, 185], OperandSize::Qword)
}

#[test]
fn vpmaxsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 207, 60, 200], OperandSize::Dword)
}

#[test]
fn vpmaxsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 680467709, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 60, 132, 83, 253, 28, 143, 40], OperandSize::Dword)
}

#[test]
fn vpmaxsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 29, 201, 60, 203], OperandSize::Qword)
}

#[test]
fn vpmaxsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 142953013, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 193, 60, 164, 192, 53, 74, 133, 8], OperandSize::Qword)
}

