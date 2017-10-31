use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 115, 211, 38], OperandSize::Dword)
}

#[test]
fn vpsrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 115, 211, 123], OperandSize::Qword)
}

#[test]
fn vpsrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 115, 211, 121], OperandSize::Dword)
}

#[test]
fn vpsrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 115, 208, 59], OperandSize::Qword)
}

#[test]
fn vpsrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 211, 240], OperandSize::Dword)
}

#[test]
fn vpsrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 27990834, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 211, 36, 245, 50, 27, 171, 1], OperandSize::Dword)
}

#[test]
fn vpsrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 211, 240], OperandSize::Qword)
}

#[test]
fn vpsrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 211, 20, 217], OperandSize::Qword)
}

#[test]
fn vpsrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 211, 223], OperandSize::Dword)
}

#[test]
fn vpsrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 934529411, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 211, 171, 131, 201, 179, 55], OperandSize::Dword)
}

#[test]
fn vpsrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 211, 203], OperandSize::Qword)
}

#[test]
fn vpsrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 295767310, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 211, 60, 125, 14, 13, 161, 17], OperandSize::Qword)
}

#[test]
fn vpsrlq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 138, 211, 224], OperandSize::Dword)
}

#[test]
fn vpsrlq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 211, 30], OperandSize::Dword)
}

#[test]
fn vpsrlq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 133, 139, 211, 234], OperandSize::Qword)
}

#[test]
fn vpsrlq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RCX, 46732407, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 213, 129, 211, 129, 119, 20, 201, 2], OperandSize::Qword)
}

#[test]
fn vpsrlq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 169, 211, 210], OperandSize::Dword)
}

#[test]
fn vpsrlq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 963831756, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 174, 211, 148, 210, 204, 231, 114, 57], OperandSize::Dword)
}

#[test]
fn vpsrlq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 133, 162, 211, 217], OperandSize::Qword)
}

#[test]
fn vpsrlq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RBX, 1796466645, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 173, 162, 211, 179, 213, 231, 19, 107], OperandSize::Qword)
}

#[test]
fn vpsrlq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 202, 211, 221], OperandSize::Dword)
}

#[test]
fn vpsrlq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 201, 211, 6], OperandSize::Dword)
}

#[test]
fn vpsrlq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 189, 207, 211, 212], OperandSize::Qword)
}

#[test]
fn vpsrlq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1033996949, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 173, 198, 211, 4, 77, 149, 138, 161, 61], OperandSize::Qword)
}

