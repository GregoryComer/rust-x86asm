use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 103, 208], OperandSize::Dword)
}

#[test]
fn vpackuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1103331152, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 103, 132, 251, 80, 127, 195, 65], OperandSize::Dword)
}

#[test]
fn vpackuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 103, 204], OperandSize::Qword)
}

#[test]
fn vpackuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 103, 47], OperandSize::Qword)
}

#[test]
fn vpackuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 103, 221], OperandSize::Dword)
}

#[test]
fn vpackuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 1762973929, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 103, 139, 233, 216, 20, 105], OperandSize::Dword)
}

#[test]
fn vpackuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 103, 244], OperandSize::Qword)
}

#[test]
fn vpackuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RBX, 1717957382, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 103, 131, 6, 243, 101, 102], OperandSize::Qword)
}

#[test]
fn vpackuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 103, 206], OperandSize::Dword)
}

#[test]
fn vpackuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 2140931750, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 103, 132, 217, 166, 6, 156, 127], OperandSize::Dword)
}

#[test]
fn vpackuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 117, 135, 103, 244], OperandSize::Qword)
}

#[test]
fn vpackuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 21, 139, 103, 9], OperandSize::Qword)
}

#[test]
fn vpackuswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 103, 224], OperandSize::Dword)
}

#[test]
fn vpackuswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 983780975, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 172, 103, 148, 78, 111, 78, 163, 58], OperandSize::Dword)
}

#[test]
fn vpackuswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 125, 173, 103, 197], OperandSize::Qword)
}

#[test]
fn vpackuswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1769122617, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 93, 162, 103, 164, 112, 57, 171, 114, 105], OperandSize::Qword)
}

#[test]
fn vpackuswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 103, 249], OperandSize::Dword)
}

#[test]
fn vpackuswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 1388198662, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 103, 152, 6, 59, 190, 82], OperandSize::Dword)
}

#[test]
fn vpackuswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 109, 206, 103, 255], OperandSize::Qword)
}

#[test]
fn vpackuswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 69, 201, 103, 33], OperandSize::Qword)
}

