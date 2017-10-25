use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsravw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 17, 230], OperandSize::Dword)
}

fn vpsravw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1929100013, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 143, 17, 172, 138, 237, 186, 251, 114], OperandSize::Dword)
}

fn vpsravw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 197, 133, 17, 229], OperandSize::Qword)
}

fn vpsravw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 189, 133, 17, 16], OperandSize::Qword)
}

fn vpsravw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 17, 217], OperandSize::Dword)
}

fn vpsravw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1486370499, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 17, 52, 157, 195, 54, 152, 88], OperandSize::Dword)
}

fn vpsravw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 197, 165, 17, 234], OperandSize::Qword)
}

fn vpsravw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 502525858, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 221, 175, 17, 180, 128, 162, 239, 243, 29], OperandSize::Qword)
}

fn vpsravw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 17, 194], OperandSize::Dword)
}

fn vpsravw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1650491058, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 17, 180, 119, 178, 126, 96, 98], OperandSize::Dword)
}

fn vpsravw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 253, 199, 17, 231], OperandSize::Qword)
}

fn vpsravw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 197, 197, 17, 34], OperandSize::Qword)
}

