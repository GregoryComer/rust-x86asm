use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 12, 38, 251], OperandSize::Dword)
}

#[test]
fn vptestmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 9, 38, 36, 201], OperandSize::Dword)
}

#[test]
fn vptestmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 205, 12, 38, 240], OperandSize::Qword)
}

#[test]
fn vptestmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 133, 14, 38, 44, 119], OperandSize::Qword)
}

#[test]
fn vptestmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 45, 38, 224], OperandSize::Dword)
}

#[test]
fn vptestmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 1046963170, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 41, 38, 144, 226, 99, 103, 62], OperandSize::Dword)
}

#[test]
fn vptestmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 197, 36, 38, 232], OperandSize::Qword)
}

#[test]
fn vptestmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RDI, 1110458785, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 141, 41, 38, 191, 161, 65, 48, 66], OperandSize::Qword)
}

#[test]
fn vptestmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 75, 38, 221], OperandSize::Dword)
}

#[test]
fn vptestmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 75, 38, 33], OperandSize::Dword)
}

#[test]
fn vptestmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 229, 65, 38, 250], OperandSize::Qword)
}

#[test]
fn vptestmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 714951054, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 165, 68, 38, 180, 208, 142, 73, 157, 42], OperandSize::Qword)
}

