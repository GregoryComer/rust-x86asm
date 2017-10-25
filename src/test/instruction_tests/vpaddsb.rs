use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 236, 217], OperandSize::Dword)
}

fn vpaddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1670068003, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 236, 52, 253, 35, 55, 139, 99], OperandSize::Dword)
}

fn vpaddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 236, 201], OperandSize::Qword)
}

fn vpaddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDX, 1635588395, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 236, 162, 43, 25, 125, 97], OperandSize::Qword)
}

fn vpaddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 236, 233], OperandSize::Dword)
}

fn vpaddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 649568359, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 236, 148, 209, 103, 160, 183, 38], OperandSize::Dword)
}

fn vpaddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 236, 208], OperandSize::Qword)
}

fn vpaddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 236, 32], OperandSize::Qword)
}

fn vpaddsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 236, 205], OperandSize::Dword)
}

fn vpaddsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 236, 52, 182], OperandSize::Dword)
}

fn vpaddsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 93, 138, 236, 240], OperandSize::Qword)
}

fn vpaddsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 300984015, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 29, 138, 236, 60, 205, 207, 166, 240, 17], OperandSize::Qword)
}

fn vpaddsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 236, 223], OperandSize::Dword)
}

fn vpaddsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 236, 4, 216], OperandSize::Dword)
}

fn vpaddsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 93, 165, 236, 199], OperandSize::Qword)
}

fn vpaddsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectDisplaced(RDI, 735362266, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 45, 171, 236, 191, 218, 188, 212, 43], OperandSize::Qword)
}

fn vpaddsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 236, 227], OperandSize::Dword)
}

fn vpaddsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ESI, 1425478313, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 236, 166, 169, 18, 247, 84], OperandSize::Dword)
}

fn vpaddsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 37, 201, 236, 230], OperandSize::Qword)
}

fn vpaddsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 117, 198, 236, 15], OperandSize::Qword)
}

