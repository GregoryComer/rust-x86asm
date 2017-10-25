use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 117, 229], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1623774639, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 117, 180, 187, 175, 213, 200, 96], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 117, 242], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 477910931, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 117, 180, 127, 147, 87, 124, 28], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 117, 242], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 254173005, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 117, 179, 77, 95, 38, 15], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 117, 248], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1080762333, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 117, 12, 213, 221, 31, 107, 64], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 13, 117, 246], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1278135431, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 12, 117, 44, 157, 135, 204, 46, 76], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 93, 14, 117, 220], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 4, 117, 15], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 41, 117, 230], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 15995341, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 44, 117, 28, 221, 205, 17, 244, 0], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 39, 117, 214], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 2142607794, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 34, 117, 156, 198, 178, 153, 181, 127], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 74, 117, 221], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 77, 117, 50], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 61, 76, 117, 240], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 53, 74, 117, 27], OperandSize::Qword)
}

